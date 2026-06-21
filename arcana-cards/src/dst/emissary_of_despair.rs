//! Emissary of Despair — `{1}{B}{B}` 2/1 black Spirit with Flying.
//! "Whenever this creature deals combat damage to a player, that
//! player loses 1 life for each artifact they control."
//!
//! Abilities:
//! 1. Flying (keyword).
//! 2. DamageDealt to a player (combat) → that player loses 1 life for
//!    each artifact they control.

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
    let name = reg.interner_mut().intern("Emissary of Despair");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
            // Scope the source filter to THIS card by name so it only
            // fires when Emissary of Despair itself deals the combat
            // damage (DamageDealt has no "this object" self-form).
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter {
                    name: Some(name),
                    ..ObjectFilter::default()
                },
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: drain_for_artifacts,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn drain_for_artifacts(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    // "for each artifact they control" — count artifacts controlled by
    // the damaged player.
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        p,
    );
    vec![Effect::LoseLife {
        player: p,
        amount: n,
    }]
}
