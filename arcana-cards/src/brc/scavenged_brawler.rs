//! Scavenged Brawler — `{6}` 4/4 Artifact Creature — Construct.
//!
//! Oracle:
//! * Flying, vigilance, trample, lifelink.
//! * `{5}, Exile this card from your graveyard: Choose target creature. Put
//!   four +1/+1 counters, a flying counter, a vigilance counter, a trample
//!   counter, and a lifelink counter on that creature. Activate only as a
//!   sorcery.`
//!
//! Keywords are base characteristics. The graveyard-activated ability pays
//! {5} and exiles this card from the graveyard, then puts the counters on a
//! targeted creature. The four +1/+1 counters use `CounterKind::PlusOnePlusOne`;
//! the keyword counters are recorded as `CounterKind::Named` (a faithful
//! counter representation — the keyword-granting behavior of those counters
//! is engine debt, but the counters are placed). Sorcery-speed via
//! `is_instant_speed: false`.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scavenged Brawler");
    let construct = reg.interner_mut().intern("Construct");
    // Pre-intern keyword-counter names so the resolver can recover them.
    let _flying = reg.interner_mut().intern("flying");
    let _vigilance = reg.interner_mut().intern("vigilance");
    let _trample = reg.interner_mut().intern("trample");
    let _lifelink = reg.interner_mut().intern("lifelink");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}, Exile this card from your graveyard: Choose target creature. Put four +1/+1 counters, a flying counter, a vigilance counter, a trample counter, and a lifelink counter on that creature. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: load_counters,
            }),
    )
}

fn load_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let id = *id;
    let mut effects = vec![Effect::AddCounters {
        target: id,
        kind: CounterKind::PlusOnePlusOne,
        count: 4,
    }];
    for nm in ["flying", "vigilance", "trample", "lifelink"] {
        if let Some(sym) = reg.interner().lookup(nm) {
            effects.push(Effect::AddCounters {
                target: id,
                kind: CounterKind::Named(sym),
                count: 1,
            });
        }
    }
    effects
}
