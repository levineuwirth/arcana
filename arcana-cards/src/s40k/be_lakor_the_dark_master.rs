//! Be'lakor, the Dark Master — `{3}{U}{B}{R}` 6/5 Legendary Creature — Demon Noble.
//!
//! Oracle:
//! * Flying
//! * Prince of Chaos — When Be'lakor enters, you draw X cards and you lose X life,
//!   where X is the number of Demons you control.
//! * Lord of Torment — Whenever another Demon you control enters, it deals damage
//!   equal to its power to any target.
//!
//! "Prince of Chaos" / "Lord of Torment" are ability-word labels, not
//! keywords. The ETB self-trigger counts Demons you control for X. The
//! Lord-of-Torment trigger fires on a Demon-you-control entering and deals
//! that creature's power to a chosen any-target. "another" cannot be
//! filtered out of a ZoneChange, so Be'lakor's own ETB also matches it
//! (minor over-fire; documented).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Be'lakor, the Dark Master");
    let demon = reg.interner_mut().intern("Demon");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(noble);

    // "another Demon you control enters" — Demon you control entering battlefield.
    let demon_enter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(demon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: prince_of_chaos,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: demon_enter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: lord_of_torment,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn prince_of_chaos(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let demon = match reg.interner().lookup("Demon") {
        Some(sym) => sym,
        None => return Vec::new(),
    };
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(demon);
    let x = script::count_matching(state, &filter, trig.controller);
    if x == 0 {
        return Vec::new();
    }
    vec![
        Effect::DrawCards { player: trig.controller, count: x },
        Effect::LoseLife { player: trig.controller, amount: x },
    ]
}

fn lord_of_torment(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(entering) = trig.entering_object() else { return Vec::new(); };
    let amount = script::power_of(state, entering).max(0) as u32;
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage { source: entering, target: dt, amount }]
}
