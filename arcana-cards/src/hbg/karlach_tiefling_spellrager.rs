//! Karlach, Tiefling Spellrager — `{1}{U}{R}` 4/4 Legendary Tiefling
//! Barbarian with First strike and Haste.
//! "When this card specializes from your graveyard, return it from your
//!  graveyard to the battlefield. It perpetually gains 'This creature
//!  can't block.'"
//! "When this card specializes from any zone, seek an instant or sorcery
//!  card with mana value 3 or less. Until end of turn, you may cast that
//!  card without paying its mana cost."

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
    let name = reg.interner_mut().intern("Karlach, Tiefling Spellrager");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Seek is not an available KeywordAbility variant.
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: specialize_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfSpecializes,
                intervening_if: None,
                effect: specialize_seek,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn specialize_from_graveyard(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Return it from your graveyard to the battlefield."
    // GAP: the "from your graveyard" qualifier and the perpetual gain of
    // "This creature can't block." are not expressible — the reanimation
    // of the source is the closest faithful piece.
    vec![Effect::ReturnFromGraveyardToBattlefield { target: trig.source }]
}

fn specialize_seek(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "seek an instant or sorcery card with mana value 3 or less" and
    // the "you may cast that card without paying its mana cost" rider have no
    // expressible Effect (Seek and free-cast permission are unmodeled).
    Vec::new()
}
