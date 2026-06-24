//! Canoptek Wraith — `{3}` 2/1 Artifact Creature — Wraith.
//! Wraith Form — This creature can't be blocked. (Static; modeled as an
//! ETB trigger applying CantBeBlocked to itself.)
//! Transdimensional Scout — When this creature deals combat damage to a
//! player, you may pay {3} and sacrifice it. If you do, choose a land you
//! control, then search your library for up to two basic land cards with
//! the same name as the chosen land, put them onto the battlefield tapped,
//! then shuffle. (GAP — COMPOUND pay-{3}-AND-sacrifice cost: OptionalPaymentKind
//! holds a single cost, not mana-plus-sacrifice; plus the choose-a-land /
//! same-name tutor has no surface.)

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Canoptek Wraith");
    let wraith = reg.interner_mut().intern("Wraith");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wraith);

    // GAP: "Transdimensional Scout" / "Wraith Form" are flavor keyword
    // labels, not KeywordAbility variants.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_unblockable,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: transdimensional_scout,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_unblockable(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::CantBeBlocked {
        target: trig.source,
        duration: Duration::WhileSourceOnBattlefield,
    }]
}

fn transdimensional_scout(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {3} and sacrifice it" is a COMPOUND optional cost
    // (mana AND a sacrifice); OptionalPaymentKind holds a single cost, so the
    // combined payment is not expressible. Then "choose a land you control"
    // and tutor up to two basics matching its name (no
    // choose-then-search-by-matching-name primitive).
    Vec::new()
}
