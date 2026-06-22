//! Concert Kaboomist — `{1}{R}{R}` 4/2 Goblin Artificer (red).
//!
//! * Trample — keyword line.
//! * "When Concert Kaboomist enters or is turned face up, it deals X damage
//!   to target opponent, where X is the number of noncreature spells you've
//!   cast since the beginning of your last turn." → wired as a
//!   `SelfEntersBattlefield` trigger that deals X damage to the targeted
//!   opponent. X is computed at resolution from the count of noncreature
//!   spells cast this turn (closest available window; "since the beginning of
//!   your last turn" is approximated by the per-turn slice). The "is turned
//!   face up" alternate cause (Disguise flip) has no matching trigger
//!   condition — only the enters half is wired.
//! * Disguise {R} — `Disguise` is not in the engine's keyword surface.
//!   GAP: Disguise.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Concert Kaboomist");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_burn_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn etb_burn_opponent(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // X = noncreature spells you've cast this turn (per-turn slice; the
    // "since the beginning of your last turn" window is approximated).
    let x = script::spells_cast_this_turn(
        state,
        &ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
        trig.controller,
    );
    if x == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(*p),
        amount: x,
    }]
}
