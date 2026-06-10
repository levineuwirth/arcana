//! Maze of Ith — nonbasic land (The Dark, 1983).
//! "{T}: Untap target attacking creature. Prevent all combat damage that
//! would be dealt to and dealt by that creature this turn." The untap and
//! the "damage dealt to" prevention are wired; the "attacking" target
//! restriction and the "dealt by" half are GAPs (see comments).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Maze of Ith");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}: Untap target attacking creature. Prevent all \
                       combat damage that would be dealt to and dealt by \
                       that creature this turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: "target attacking creature" — the attacking
                // restriction is not expressible in this target catalog;
                // any creature may be targeted.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: maze_out,
            },
        ),
    )
}

fn maze_out(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: prevention is combat-damage-only on the card; PreventDamage
    // prevents all damage to the creature this turn (over-broad).
    // GAP: "and dealt by that creature" — source-specific prevention keyed
    // to one object id is not expressible (PreventDamageFrom takes an
    // ObjectFilter, not an id); only the dealt-to half is wired.
    vec![
        Effect::Untap { target: *id },
        Effect::PreventDamage {
            target: DamageTarget::Object(*id),
            amount: None,
            duration: ReplacementDuration::EndOfTurn,
        },
    ]
}
