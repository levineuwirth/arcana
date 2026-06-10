//! Runesword — `{6}` artifact (Chronicles, 1995).
//! "{3}, {T}: Target attacking creature gets +2/+0 until end of turn.
//! When that creature leaves the battlefield this turn, sacrifice this
//! artifact. If the creature deals damage to a creature this turn, the
//! creature dealt damage can't be regenerated this turn. If a creature
//! dealt damage by the targeted creature would die this turn, exile
//! that creature instead."
//! The pump is wired; the leave-the-battlefield sacrifice rider, the
//! can't-be-regenerated rider, and the exile-instead-of-die replacement
//! are GAPs (see comments in the resolver).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Runesword");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{3}, {T}: Target attacking creature gets +2/+0 until \
                       end of turn. When that creature leaves the battlefield \
                       this turn, sacrifice this artifact. If the creature \
                       deals damage to a creature this turn, the creature \
                       dealt damage can't be regenerated this turn. If a \
                       creature dealt damage by the targeted creature would \
                       die this turn, exile that creature instead."
                    .into(),
                // GAP: "target ATTACKING creature" — ObjectFilter has no
                // attacking/combat-status predicate in this API surface;
                // wired as plain target creature.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_attacker,
            },
        ),
    )
}

fn pump_attacker(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // GAP: "When that creature leaves the battlefield this turn, sacrifice
    // this artifact" — no delayed trigger on a target's zone change.
    // GAP: "the creature dealt damage can't be regenerated this turn" — no
    // regeneration-forbid effect.
    // GAP: "would die this turn, exile that creature instead" — dies-to-exile
    // replacement scoped to damage from the target is not expressible.
    vec![Effect::Pump {
        target: *id,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
