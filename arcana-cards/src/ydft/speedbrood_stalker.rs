//! Speedbrood Stalker — `{3}{B}{B}` 3/4 Insect Assassin.
//!
//! Flying, Lifelink.
//! When this creature enters, choose target opponent. Secretly choose a
//! creature or planeswalker that player controls. That player sacrifices a
//! creature or planeswalker of their choice, then sacrifices the chosen
//! permanent.
//!
//! Flying and Lifelink are base keywords. The ETB targets an opponent and
//! makes them sacrifice a creature or planeswalker of their choice (the
//! "of their choice" edict). The secret-choice mechanic and the *second*
//! sacrifice (of the secretly chosen permanent) are GAP'd — they cannot be
//! expressed with the available primitives.

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Speedbrood Stalker");
    let insect = reg.interner_mut().intern("Insect");
    let assassin = reg.interner_mut().intern("Assassin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    subtypes.0.insert(assassin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_edict,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
        }),
    )
}

fn etb_edict(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Player(p) = target else {
        return Vec::new();
    };
    // GAP: the secret-choice and the second "then sacrifices the chosen
    // permanent" sacrifice are unmodeled; only the "sacrifices a creature or
    // planeswalker of their choice" edict is wired.
    vec![Effect::Sacrifice {
        player: *p,
        filter: ObjectFilter::new()
            .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::PLANESWALKER)),
        count: 1,
    }]
}
