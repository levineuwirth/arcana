//! Elegy Acolyte — `{2}{B}{B}` 4/4 Creature — Human Cleric.
//!
//! Oracle:
//! * Lifelink.
//! * "Whenever one or more creatures you control deal combat damage to
//!   a player, you draw a card and lose 1 life." → a combat-damage
//!   trigger (creatures you control → a player). The "one or more …
//!   as a batch" once-per-combat nuance is a fidelity simplification —
//!   the engine fires per damaging creature.
//! * "Void — At the beginning of your end step, if a nonland permanent
//!   left the battlefield this turn or a spell was warped this turn,
//!   create a 2/2 colorless Robot artifact creature token." → Void is
//!   not a usable keyword, and its intervening-if condition ("a nonland
//!   permanent left the battlefield / a spell was warped this turn") has
//!   no `conditions::` helper, so the whole gated trigger is a GAP
//!   rather than firing unconditionally.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP keyword: Void is not in the usable KeywordAbility surface.
// GAP trigger: the Void end-step token ability has an intervening-if
//   ("a nonland permanent left the battlefield this turn or a spell was
//   warped this turn") with no available condition predicate.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elegy Acolyte");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: draw_and_lose,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn draw_and_lose(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: trig.controller, count: 1 },
        Effect::LoseLife { player: trig.controller, amount: 1 },
    ]
}
