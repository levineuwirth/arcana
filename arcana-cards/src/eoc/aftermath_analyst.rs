//! Aftermath Analyst — `{1}{G}` 1/3 Elf Detective.
//! "When this creature enters, mill three cards." plus
//! "{3}{G}, Sacrifice this creature: Return all land cards from your
//! graveyard to the battlefield tapped."
//!
//! The Mill keyword line is reminder text for the ETB; the engine has
//! no standalone Mill keyword to attach, so `keywords` is empty and the
//! mill is wired as the ETB effect. The sac-activation's batch
//! graveyard return is not expressible (no enumeration of graveyard ids).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aftermath Analyst");
    let elf = reg.interner_mut().intern("Elf");
    let detective = reg.interner_mut().intern("Detective");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(detective);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_mill_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{G}, Sacrifice this creature: Return all land cards from your graveyard to the battlefield tapped.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{G}").expect("valid cost"),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: return_all_lands,
            }),
    )
}

fn etb_mill_three(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Mill { player: trig.controller, count: 3 }]
}

fn return_all_lands(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Return ALL land cards from your graveyard to the battlefield tapped" —
    // no batch graveyard-return effect and no graveyard id enumeration in script::;
    // Reanimate returns a single chosen card and ReturnFromGraveyardToBattlefield is
    // targeted/single. Cannot enumerate "all lands" in the graveyard.
    Vec::new()
}
