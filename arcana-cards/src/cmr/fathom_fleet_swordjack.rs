//! Fathom Fleet Swordjack — `{3}{R}` 4/3 Orc Pirate.
//!
//! Oracle:
//! * Whenever this creature attacks, it deals damage to the player or
//!   planeswalker it's attacking equal to the number of artifacts you control.
//! * Encore {5}{R}.
//!
//! GAP: Encore is not in the usable keyword surface; its graveyard
//! token-copy-per-opponent activation is omitted.
//! GAP (partial): the attack trigger's target is "the player OR PLANESWALKER
//! it's attacking" — only the defending-player case is reachable via
//! `trig.defending_player()`; the planeswalker-attack case deals nothing.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fathom Fleet Swordjack");
    let orc = reg.interner_mut().intern("Orc");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_damage_per_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn attack_damage_per_artifact(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else {
        return Vec::new();
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::new()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(p),
        amount: n,
    }]
}
