//! Hobgoblin Bandit Lord — `{1}{R}{R}` 2/3 Creature — Goblin Rogue.
//!
//! Oracle:
//! * "Other Goblins you control get +1/+1." — a static anthem/lord effect (no
//!   trigger word, no cost); not expressible as a triggered/activated ability,
//!   GAP'd.
//! * "{R}, {T}: This creature deals damage equal to the number of Goblins that
//!   entered the battlefield under your control this turn to any target." — a
//!   mana + tap activation; the dynamic amount is the count of Goblins that
//!   entered under your control this turn.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: static — "Other Goblins you control get +1/+1." Anthem/lord effect not
// expressible as a triggered/activated ability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hobgoblin Bandit Lord");
    let goblin = reg.interner_mut().intern("Goblin");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(rogue);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{R}, {T}: This creature deals damage equal to the number of Goblins that entered the battlefield under your control this turn to any target.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{R}").expect("valid cost"),
                tap: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::any_target()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: deal_per_goblin_entered,
        }),
    )
}

fn deal_per_goblin_entered(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    let n = script::entered_this_turn_matching(
        state,
        &script::subtype_filter(reg, "Goblin").controlled_by(ControllerConstraint::You),
        ctx.controller,
    );
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: n,
    }]
}
