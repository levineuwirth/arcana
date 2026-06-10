//! Sentinel Tower — `{4}` artifact (Battlebond).
//! "Whenever an instant or sorcery spell is cast during your turn,
//! this artifact deals damage to any target equal to 1 plus the
//! number of instant and sorcery spells cast before that spell this
//! turn." The trigger and scaling damage are wired with two
//! documented GAPs: the during-your-turn gate is not expressible on
//! SpellCast, and the this-turn spell count helper is scoped to the
//! controller rather than all players.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sentinel Tower");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                // GAP: trigger — "during your turn" is not expressible on
                // TriggerCondition::SpellCast; fires on any player's
                // instant/sorcery cast in any turn.
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::new().with_types_any(TypeLine(
                        TypeLine::INSTANT | TypeLine::SORCERY,
                    ))),
                    caster: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: deal_scaling_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            },
        ),
    )
}

fn deal_scaling_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(choice) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let target = match choice {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    // 1 + spells cast BEFORE the triggering spell this turn; the triggering
    // spell is already in the turn slice, so the total count IS that value.
    // GAP: script::spells_cast_this_turn is scoped to the ability's
    // controller; the oracle counts all players' instants/sorceries.
    let filter = ObjectFilter::new()
        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));
    let amount =
        script::spells_cast_this_turn(state, &filter, trig.controller).max(1);
    vec![Effect::DealDamage { target, amount, source: trig.source }]
}
