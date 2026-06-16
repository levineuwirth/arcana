//! Gor Muldrak, Amphinologist — `{1}{G}{U}` 3/2 Legendary Creature — Human Scout (G/U).
//!
//! * "You and permanents you control have protection from Salamanders." — a
//!   pure static protection-granting ability; Protection is not an available
//!   `KeywordAbility` variant and there is no Effect for board-wide protection,
//!   so it is GAP'd (static, not a triggered/activated ability).
//! * "At the beginning of your end step, each player who controls the fewest
//!   creatures creates a 4/3 blue Salamander Warrior creature token." — wired
//!   as an end-step trigger; the per-player "fewest creatures" comparison
//!   cannot be expressed with the demonstrated script helpers, so the effect
//!   is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

// GAP: static — "You and permanents you control have protection from
// Salamanders." (Protection is not an expressible KeywordAbility / Effect.)

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gor Muldrak, Amphinologist");
    let human = reg.interner_mut().intern("Human");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: fewest_creatures_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn fewest_creatures_token(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each player who controls the fewest creatures creates a token" —
    // per-player minimum-creature comparison is not expressible with the
    // available script helpers.
    Vec::new()
}
