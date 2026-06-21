//! Nashi, Searcher in the Dark — `{U}{B}` 2/2 Legendary Rat Ninja
//! Wizard with Menace.
//!
//! * Menace — keyword line. ("Mill" in the Scryfall keyword list is the
//!   triggered effect's verb, not a usable `KeywordAbility` variant.)
//! * "Whenever Nashi deals combat damage to a player, you mill that many
//!   cards. You may put any number of legendary and/or enchantment cards
//!   from among them into your hand. If you put no cards into your hand
//!   this way, put a +1/+1 counter on Nashi." → a `DamageDealt` trigger
//!   (this creature → a player, combat-only). You mill that many cards
//!   (`Effect::Mill` with the dynamic damage amount). GAP: "put any
//!   number of legendary/enchantment cards from among them into your
//!   hand" (no select-from-just-milled primitive) and the dependent
//!   "if you put no cards into your hand, put a +1/+1 counter on Nashi"
//!   (depends on that selection) — both omitted; only the mill fires.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nashi, Searcher in the Dark");
    let rat = reg.interner_mut().intern("Rat");
    let ninja = reg.interner_mut().intern("Ninja");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(ninja);
    subtypes.0.insert(wizard);

    let self_name = reg.interner().lookup("Nashi, Searcher in the Dark");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter {
                        name: self_name,
                        ..ObjectFilter::default()
                    },
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn combat_damage_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "you mill that many cards" — that many = the combat damage dealt.
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::Mill {
        player: trig.controller,
        count: n,
    }]
}
