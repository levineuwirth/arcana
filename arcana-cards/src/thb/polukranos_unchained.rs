//! Polukranos, Unchained — `{2}{B}{G}` 0/0 Legendary Creature — Zombie Hydra.
//! Enters with six +1/+1 counters on it. (Escape-with-twelve variant
//! GAP'd — no escape-aware enters-with spec.)
//! If damage would be dealt to Polukranos while it has a +1/+1 counter
//! on it, prevent that damage and remove that many +1/+1 counters from
//! it. (Static replacement — GAP'd.)
//! {1}{B}{G}: Polukranos fights another target creature.
//! Escape—{4}{B}{G}, Exile six other cards from your graveyard. (Escape
//! cast cost — GAP'd: not expressible.)
//! Keywords Fight/Escape are not in the supported KeywordAbility surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Polukranos, Unchained");
    let zombie = reg.interner_mut().intern("Zombie");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: keywords Fight / Escape not in supported surface.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static replacement "if damage would be dealt while it has a
    // +1/+1 counter, prevent and remove that many counters" — no Effect.
    // GAP: Escape cast cost ({4}{B}{G}, exile six other graveyard cards;
    // escapes with twelve counters) — not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::PlusOnePlusOne,
                count: 6,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{G}: Polukranos fights another target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fight_target,
            }),
    )
}

fn fight_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::Fight { a: ctx.source, b: *id }]
}
