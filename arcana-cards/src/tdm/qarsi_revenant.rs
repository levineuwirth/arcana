//! Qarsi Revenant — `{1}{B}{B}` 3/3 Creature — Vampire.
//! Flying, deathtouch, lifelink.
//! "Renew — {2}{B}, Exile this card from your graveyard: Put a flying counter,
//!  a deathtouch counter, and a lifelink counter on target creature. Activate
//!  only as a sorcery."
//!
//! Three evergreen keywords plus a graveyard-activated Renew ability that
//! exiles this card to place three keyword counters (modeled as named
//! counters) on a target creature.

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
    let name = reg.interner_mut().intern("Qarsi Revenant");
    let vampire = reg.interner_mut().intern("Vampire");
    // Pre-intern the keyword-counter names so the resolver lookups succeed.
    let _flying = reg.interner_mut().intern("flying");
    let _deathtouch = reg.interner_mut().intern("deathtouch");
    let _lifelink = reg.interner_mut().intern("lifelink");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Renew — {2}{B}, Exile this card from your graveyard: Put a flying counter, a deathtouch counter, and a lifelink counter on target creature. Activate only as a sorcery.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{2}{B}").expect("valid cost"),
                exile_self: true,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Graveyard,
            is_instant_speed: false,
            face_gate: None,
            effect: put_keyword_counters,
        }),
    )
}

fn put_keyword_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let mut effects = Vec::new();
    for nm in ["flying", "deathtouch", "lifelink"] {
        if let Some(sym) = reg.interner().lookup(nm) {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::Named(sym),
                count: 1,
            });
        }
    }
    effects
}
