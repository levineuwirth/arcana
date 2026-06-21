//! Quicksilver, Brash Blur — `{R}` 1/1 Legendary Mutant Hero.
//! If in your opening hand, you may begin the game with him on the battlefield.
//! Haste.
//! Power-up — {4}{R}: Put a +1/+1 counter and a double strike counter on
//! Quicksilver. (Activate each power-up ability only once. Reduce the cost by
//! his mana cost if he entered this turn.)
//!
//! Haste is a base characteristic. The "begin the game with him on the
//! battlefield" static is a start-of-game replacement with no primitive → GAP.
//! The Power-up activated ability puts a +1/+1 counter and a "double strike"
//! named counter on Quicksilver; the once_per_turn field gates it (closest to
//! "activate only once" — really once per game, GAPped) and the
//! "reduce the cost by his mana cost if he entered this turn" rider is GAPped
//! (no dynamic cost reduction).

use arcana_core::effects::Effect;
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quicksilver, Brash Blur");
    let mutant = reg.interner_mut().intern("Mutant");
    let hero = reg.interner_mut().intern("Hero");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(hero);
    let _double_strike = reg.interner_mut().intern("double strike");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    // GAP: "If Quicksilver is in your opening hand, you may begin the game with
    // him on the battlefield" — start-of-game replacement, no primitive.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Power-up — {4}{R}: Put a +1/+1 counter and a double strike counter on Quicksilver.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{4}{R}").expect("valid cost"),
                // GAP: "Activate each power-up ability only once" is once-per-
                // GAME; once_per_turn is the closest gate (over-permits across
                // turns). "Reduce the cost by his mana cost if he entered this
                // turn" — no dynamic cost reduction, GAPped.
                once_per_turn: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: power_up,
        }),
    )
}

fn power_up(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut out = vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(ds) = reg.interner().lookup("double strike") {
        out.push(Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Named(ds),
            count: 1,
        });
    }
    out
}
