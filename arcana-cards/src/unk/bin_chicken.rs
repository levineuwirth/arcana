//! Bin Chicken — `{3}{B}{B}` 3/2 Bird Pest.
//!
//! * "You may treat Food tokens as though they were Junk tokens and vice
//!   versa." — GAP: pure static token-type substitution; not expressible,
//!   and Food is not a usable keyword variant (`keywords: vec![]`).
//! * "When this creature enters, choose one — Create a Food Token. / Amass
//!   Birds X, where X is the number of Foods you control." — GAP: a modal
//!   "choose one" player choice is not expressible on a triggered ability
//!   (modal dispatch is a spell-ability-only facility here). The ETB
//!   trigger is wired with an empty effect.
//! * "{3}{B}{B}, Exile an artifact card from your graveyard: Return this
//!   card from your graveyard to the battlefield." — wired as a
//!   graveyard-activated ability returning this card; GAP: the additional
//!   "exile an artifact card from your graveyard" cost has no ActivationCost
//!   field for exiling another card from the graveyard.

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
    let name = reg.interner_mut().intern("Bin Chicken");
    let bird = reg.interner_mut().intern("Bird");
    let pest = reg.interner_mut().intern("Pest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(pest);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_choose_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}, Exile an artifact card from your graveyard: Return this card from your graveyard to the battlefield.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_self_from_graveyard,
            }),
    )
}

fn etb_choose_one(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: modal "choose one" is not expressible on a triggered ability.
    Vec::new()
}

fn return_self_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
