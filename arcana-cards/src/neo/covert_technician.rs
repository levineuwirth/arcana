//! Covert Technician — `{2}{U}` 2/4 Artifact Creature — Human Ninja.
//! Ninjutsu {1}{U}.
//! "Whenever this creature deals combat damage to a player, you may put an
//!  artifact card with mana value less than or equal to that damage from your
//!  hand onto the battlefield."
//!
//! Ninjutsu (an alternative cast/return mechanic) is not in the usable
//! KeywordAbility surface and has no cost field — GAP'd. The combat-damage
//! trigger is wired as a hand→battlefield put of an artifact card; the
//! "mana value ≤ that damage" cap is a fidelity GAP (no dynamic-cmc filter from
//! the damage amount is expressible — any artifact in hand is offered instead).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Covert Technician");
    let human = reg.interner_mut().intern("Human");
    let ninja = reg.interner_mut().intern("Ninja");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(ninja);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: keyword — Ninjutsu not in the usable KeywordAbility surface;
        // no alternative-cost field. Omitted.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: put_artifact_from_hand,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn put_artifact_from_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP (fidelity): "mana value ≤ that damage" cap not expressible from the
    // damage amount; any artifact card in hand is offered.
    vec![Effect::PutFromHandOntoBattlefield {
        player: trig.controller,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        tapped: false,
    }]
}
