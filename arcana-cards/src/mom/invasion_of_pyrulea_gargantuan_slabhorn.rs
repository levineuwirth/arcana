//! Invasion of Pyrulea // Gargantuan Slabhorn — `{G}{U}` Battle — Siege.
//! Enters with defense counters.
//! When this Siege enters, scry 3, then reveal the top card of your library. If it's a land or
//! double-faced card, draw a card.
//! Back face (Gargantuan Slabhorn): Creature — Beast with Trample, ward {2}, and
//! "Other transformed permanents you control have trample and ward {2}."
//!
//! GAP: defeat→cast-back-face not auto-wired (CR 310.11).
//! GAP: ETB reveal "if it's a land or double-faced card, draw a card" — conditional-on-revealed-
//!      card-type is not expressible; we perform the scry 3 only and omit the reveal/draw clause.
//! GAP: back-face static "Other transformed permanents you control have trample and ward {2}" —
//!      no static-ability mechanism for granting keywords to a filtered set of permanents.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Invasion of Pyrulea");
    let siege_sub = reg.interner_mut().intern("Siege");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(siege_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::BATTLE.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Gargantuan Slabhorn — Creature — Beast with Trample, ward {2}.
    let back_name = reg.interner_mut().intern("Gargantuan Slabhorn");
    let beast_sub = reg.interner_mut().intern("Beast");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(beast_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green() | ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(7)),
            toughness: Some(PtValue::Fixed(7)),
            keywords: vec![
                KeywordAbility::Trample,
                KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
            ],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Defense,
                count: 5,
            })
            .with_transform_back(back)
            // "When this Siege enters, scry 3, then reveal the top card of your library.
            //  If it's a land or double-faced card, draw a card."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: defeat→cast-back-face not auto-wired (CR 310.11).
        // GAP: back-face static granting trample + ward {2} to other transformed permanents.
    )
}

fn etb_scry(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the "reveal top card; if it's a land or double-faced card, draw a card" rider is not
    //      expressible (no conditional-on-revealed-card-type effect). Scry 3 only.
    vec![Effect::Scry {
        player: trig.controller,
        count: 3,
    }]
}
