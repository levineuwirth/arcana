//! Hollowhenge Wrangler — `{3}{G}{G}` 6/6 Creature — Elemental.
//! "When Hollowhenge Wrangler enters the battlefield, seek a land card."
//! "Discard a land card: Conjure a card named Hollowhenge Beast into your hand.
//!  You may also activate this ability while Hollowhenge Wrangler is in your
//!  graveyard."
//!
//! Seek and Conjure are Alchemy/Arena-only mechanics with no engine effect,
//! so both ability effects are GAP'd. The activated ability's discard-a-land
//! cost is modeled faithfully. (The "also activatable from your graveyard"
//! permission is noted as a partial — a single activation_zone is declared.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hollowhenge Wrangler");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_seek_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Partial: "You may also activate this ability while ~ is in your
            // graveyard" — only the battlefield activation zone is declared.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a land card: Conjure a card named Hollowhenge Beast into your hand.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter {
                        types: Some(TypeLine::LAND.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: conjure_beast,
            }),
    )
}

fn etb_seek_land(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
    // GAP: "seek a land card" — Seek is an Alchemy/Arena-only mechanic with no
    // engine effect.
}

fn conjure_beast(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    Vec::new()
    // GAP: Conjure not modeled (Arena-only mechanic; would need
    // registry-by-name lookup in Effect::execute).
}
