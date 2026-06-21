//! Defiler of Instinct — `{2}{R}{R}` 4/4 Phyrexian Kavu with First strike.
//! First strike.
//! "As an additional cost to cast red permanent spells, you may pay 2
//! life. Those spells cost {R} less to cast if you paid life this way."
//! (a cost-altering static — GAP'd.)
//! Whenever you cast a red permanent spell, this creature deals 1 damage
//! to any target.
//!
//! First strike is a base keyword. The optional-additional-cost /
//! cost-reduction static has no expressible primitive and is GAP'd. The
//! cast trigger fires on your red permanent spells (a red, non-instant-
//! non-sorcery spell filter) and deals 1 damage to any target.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Defiler of Instinct");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(kavu);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP (static): "As an additional cost to cast red permanent spells, you
    // may pay 2 life. Those spells cost {R} less if you paid life this way."
    // No cost-altering / optional-additional-cost primitive.

    let red_permanent = ObjectFilter::new()
        .with_colors(ColorSet::red())
        .without_types(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY));

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SpellCast {
                filter: Some(red_permanent),
                caster: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: ping_any_target,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement::any_target()],
        }),
    )
}

fn ping_any_target(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 1,
    }]
}
