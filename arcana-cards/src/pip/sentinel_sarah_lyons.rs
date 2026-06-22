//! Sentinel Sarah Lyons — `{3}{R}{W}` 4/4 Legendary Human Knight.
//! Haste.
//! "As long as an artifact entered the battlefield under your control this
//!   turn, creatures you control get +2/+2." — GAP (static continuous
//!   conditional anthem; not a triggered/activated ability).
//! Battalion — "Whenever Sentinel Sarah Lyons and at least two other
//!   creatures attack, Sentinel Sarah Lyons deals damage equal to the
//!   number of artifacts you control to target player."
//!
//! The Battalion trigger is modeled with SelfAttacks; the "and at least
//! two other creatures attack" rider (a count of co-attackers) has no
//! TriggerCondition expression — // GAP: Battalion co-attacker gate (over-
//! fires whenever this attacks). The damage amount IS dynamic (number of
//! artifacts you control), computed via script.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sentinel Sarah Lyons");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static "As long as an artifact entered the battlefield under
    // your control this turn, creatures you control get +2/+2." — a
    // conditional continuous anthem, not a triggered/activated ability.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: battalion_damage,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::target_player()],
        }),
    )
}

fn battalion_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Battalion's "and at least two other creatures attack" co-
    // attacker gate is not expressible — over-fires whenever this attacks.
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let n = script::count_matching(
        state,
        &ObjectFilter::permanent()
            .with_types(TypeLine::ARTIFACT.into())
            .controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(*p),
        amount: n,
    }]
}
