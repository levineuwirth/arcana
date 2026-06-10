//! Island of Wak-Wak — nonbasic land (Arabian Nights).
//! "{T}: Target creature with flying has base power 0 until end of turn."
//! The base-power set is modeled with `SetBasePT`, preserving the
//! creature's current toughness (read at resolution via
//! `script::toughness_of`); the 'with flying' target restriction is a
//! documented gap.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Island of Wak-Wak");
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
                text: "{T}: Target creature with flying has base power 0 \
                       until end of turn."
                    .into(),
                cost: ActivationCost::tap_only(),
                // GAP: 'target creature WITH FLYING' — ObjectFilter has no
                // keyword predicate in the demonstrated surface; any
                // creature is targetable.
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: ground_power,
            },
        ),
    )
}

fn ground_power(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // Oracle sets base POWER only; SetBasePT sets both, so the current
    // toughness is read at resolution and re-applied (fidelity
    // approximation for later toughness changes).
    let toughness = script::toughness_of(state, *id);
    vec![Effect::SetBasePT {
        target: *id,
        power: 0,
        toughness,
        duration: Duration::EndOfTurn,
    }]
}
