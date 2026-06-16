//! Yennett, Cryptic Sovereign — `{2}{W}{U}{B}` 3/5 Legendary Sphinx.
//! Flying, vigilance, menace.
//! "Whenever Yennett attacks, reveal the top card of your library. You
//! may cast it without paying its mana cost if its mana value is odd.
//! If you don't cast it, draw a card." — the reveal + conditional
//! free-cast-if-odd-mana-value is not expressible (no primitive casts
//! the top card by parity). The "if you don't cast it, draw a card"
//! fallback is emitted as a best-effort partial; the conditional cast
//! is GAP'd.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Yennett, Cryptic Sovereign");
    let sphinx = reg.interner_mut().intern("Sphinx");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sphinx);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{U}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: attack_reveal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn attack_reveal(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "reveal the top card; you may cast it for free if its mana
    // value is odd" — no primitive casts the top card by mana-value
    // parity. The "if you don't cast it, draw a card" fallback is
    // emitted unconditionally as a best-effort partial.
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}
