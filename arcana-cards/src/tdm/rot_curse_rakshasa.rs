//! Rot-Curse Rakshasa — `{1}{B}` 5/5 Demon (black).
//!
//! Oracle:
//! * "Trample" — keyword.
//! * "Decayed (This creature can't block. When it attacks, sacrifice it at end
//!   of combat.)" — Decayed is not in the usable keyword surface (no
//!   `KeywordAbility::Decayed`). GAP'd: both the "can't block" and the
//!   "sacrifice at end of combat" riders.
//! * "Renew — {X}{B}{B}, Exile this card from your graveyard: Put a decayed
//!   counter on each of X target creatures. Activate only as a sorcery." — Renew
//!   is a graveyard activated ability. Wired with cost `{X}{B}{B}` + exile_self,
//!   activated from the graveyard at sorcery speed, X target creatures each
//!   receiving a "decayed" (named) counter. (Renew as a keyword marker is not
//!   available; the activated ability carries the behavior.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rot-Curse Rakshasa");
    let demon = reg.interner_mut().intern("Demon");
    // Pre-intern the named counter so the resolver can recover it.
    let _decayed = reg.interner_mut().intern("decayed");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);

    // GAP: keyword "Decayed" — not in the usable keyword surface; its "can't
    // block" and "sacrifice at end of combat when it attacks" riders are GAP'd.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Renew — {X}{B}{B}, Exile this card from your graveyard: Put a decayed counter on each of X target creatures. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{X}{B}{B}").expect("valid cost"),
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::X,
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: renew_decayed,
            }),
    )
}

fn renew_decayed(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(decayed) = reg.interner().lookup("decayed") else { return Vec::new(); };
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::Named(decayed),
                count: 1,
            }),
            _ => None,
        })
        .collect()
}
