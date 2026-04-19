//! Chandra, Pyromaster — `{2}{R}{R}` planeswalker, starting loyalty 4.
//! M15 mythic. The seed's touchstone for CR 606 loyalty abilities:
//! `+1`-cost (add Loyalty counter) and the once-per-turn-per-PW
//! activation rule.
//!
//! # Rules references
//!
//! * CR 113.3c — entering the battlefield with loyalty counters equal
//!   to printed loyalty.
//! * CR 606 — loyalty abilities. Activated abilities whose cost is
//!   adding or removing loyalty counters from the source permanent.
//! * CR 606.3 — only the PW's controller, only at sorcery speed with
//!   the stack empty, only one per turn per planeswalker.
//! * CR 704.5i — a planeswalker with 0 loyalty goes to its owner's
//!   graveyard as a state-based action.
//!
//! # Scope
//!
//! Only the `+1` ability is modeled here: "Chandra, Pyromaster deals 1
//! damage to target player." The `0` ability ("exile the top card of
//! your library, you may play it this turn") requires the
//! "cast-from-exile" modifier plumbing, and the `−7` ultimate ("you
//! get an emblem") requires emblem creation — both deferred. The
//! loyalty-pipeline wiring is what this card is seed-proving; adding
//! the rest of Chandra's abilities is mechanical once those
//! primitives land.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Pyromaster");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        // Printed starting loyalty. `after_enter_battlefield` reads
        // this and places the corresponding Loyalty counters via
        // place_counters (CR 113.3c).
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Chandra, Pyromaster deals 1 damage to target \
                       player.".into(),
                cost: ActivationCost {
                    // +1 loyalty: engine emits an AddCounters
                    // additional-cost payment that routes through
                    // place_counters, so Doubling Season et al. stack.
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                effect: plus_one_damage,
            }),
    )
}

/// `+1: Chandra, Pyromaster deals 1 damage to target player.`
fn plus_one_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}
