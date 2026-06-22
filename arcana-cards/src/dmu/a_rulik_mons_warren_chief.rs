//! A-Rulik Mons, Warren Chief — `{1}{R}{G}{G}` 4/4 Legendary Goblin.
//!
//! * Menace.
//! * Whenever Rulik Mons enters or attacks, look at the top card of
//!   your library. If it's a land card, you may put it onto the
//!   battlefield tapped. If you didn't put a card onto the battlefield
//!   this way, create a 1/1 red Goblin creature token. — This is a
//!   look-at-top / conditional-land-put-onto-battlefield / else-make-a-
//!   token cascade; the demonstrated primitives (`DigTopN` puts the
//!   chosen card into HAND, not the battlefield, and there is no
//!   "did-you-put" conditional gate). GAP'd: the two trigger shells are
//!   emitted, but their effect bodies are empty.

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
    let name = reg.interner_mut().intern("A-Rulik Mons, Warren Chief");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: look_then_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: look_then_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn look_then_token(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top card; if a land you may put it onto the
    // battlefield tapped; if you didn't put a card this way, create a
    // 1/1 red Goblin token." No primitive puts a known top card onto the
    // battlefield with a may-choice, nor gates the token on whether the
    // put happened.
    Vec::new()
}
