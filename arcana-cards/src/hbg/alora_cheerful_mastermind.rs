//! Alora, Cheerful Mastermind — `{3}{W}{U}` 4/4 legendary white-blue creature.
//! "Whenever you attack, up to one target attacking creature can't be blocked
//! this turn. At the beginning of the next end step, return that creature to its
//! owner's hand. If you do, create a 1/1 white Soldier creature token."
//!
//! GAP: trigger — "whenever you attack" maps to CreatureAttacks with self-
//! controlled filter; using that.
//! GAP: effect — "return that creature at next end step then create Soldier" is
//! a delayed conditional; emitting ForbidAttacking + ReturnToHand + CreateToken
//! immediately as best-effort.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::layers::Duration;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Alora, Cheerful Mastermind");
    let halfling = reg.interner_mut().intern("Halfling");
    let rogue = reg.interner_mut().intern("Rogue");
    let _soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(halfling);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: attack_unblockable_bounce_soldier,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: Some(ControllerConstraint::You),
                }],
            }),
    )
}

fn attack_unblockable_bounce_soldier(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let soldier = reg.interner().lookup("Soldier")
        .expect("Soldier interned during register()");
    let mut st = SubtypeSet::default();
    st.0.insert(soldier);
    let token = TokenDefinition {
        name: soldier,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: st,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: effect — "can't be blocked" (unblockable until turn) has no direct Effect variant;
    // using ForbidAttacking as closest available
    // GAP: effect — return at next end step then create token (conditional delayed)
    vec![
        Effect::ForbidAttacking { target: *id, duration: Duration::EndOfTurn },
        Effect::ReturnToHand { target: *id },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}
