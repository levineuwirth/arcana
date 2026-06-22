//! Hidetsugu and Kairi — `{2}{U}{U}{B}` 5/4 Legendary Ogre Demon
//! Dragon with Flying.
//! "When ~ enters, draw three cards, then put two cards from your hand
//!  on top of your library in any order.
//!  When ~ dies, exile the top card of your library. Target opponent
//!  loses life equal to its mana value. If it's an instant or sorcery
//!  card, you may cast it without paying its mana cost."
//!
//! ETB: the "draw three cards" half is wired faithfully.
//! GAP: "then put two cards from your hand on top of your library in
//! any order" — no primitive places chosen hand cards on top of the
//! library.
//! GAP: the entire dies effect depends on exiling the top library card
//! and reading its mana value (lose-life-equal-to-mv) plus a
//! conditional free-cast — there is no primitive that exiles the top
//! card and exposes its mana value to a downstream effect, so the
//! dies trigger's resolver is a no-op.

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
    let name = reg.interner_mut().intern("Hidetsugu and Kairi");
    let ogre = reg.interner_mut().intern("Ogre");
    let demon = reg.interner_mut().intern("Demon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ogre);
    subtypes.0.insert(demon);
    subtypes.0.insert(dragon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_draw_three,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_exile,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_draw_three(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "then put two cards from your hand on top of your library in
    // any order" — no hand-to-library-top primitive.
    vec![Effect::DrawCards { player: trig.controller, count: 3 }]
}

fn dies_exile(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: exile top card of library + target opponent loses life
    // equal to its mana value + conditional free-cast — no primitive
    // exposes the exiled card's mana value to a downstream effect.
    Vec::new()
}
