//! Cartographer's Hawk — `{1}{W}` 2/1 Bird with Flying.
//! When this creature deals combat damage to a player who controls more lands
//! than you, return it to its owner's hand. If you do, you may search your
//! library for a Plains card, put it onto the battlefield tapped, then shuffle.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cartographer's Hawk");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // Self combat-damage trigger; ObjectFilter has no per-source "this
            // object" predicate, so the source is scoped to creatures you
            // control (the closest available constraint). The residual
            // over-fire on other creatures you control is a documented gap.
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new()
                    .controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: bounce_and_tutor_plains,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn bounce_and_tutor_plains(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "to a player who controls more lands than you" gate cannot be
    // expressed (no two-player land-count comparison helper); the bounce + Plains
    // search payoff is emitted unconditionally.
    let plains_filter = script::subtype_filter(reg, "Plains");
    vec![Effect::Sequence(vec![
        Effect::ReturnToHand {
            target: trig.source,
        },
        Effect::TutorToBattlefield {
            player: trig.controller,
            filter: plains_filter,
            tapped: true,
        },
    ])]
}
