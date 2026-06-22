//! Windy City Aven — `{2}{U}{U}` 4/3 Bird Warrior with Flying.
//! Ascend MagicCon: Chicago (a bespoke city's-blessing trigger).
//! Whenever this creature attacks, if you have the Windy City's blessing,
//! create a 1/1 blue Bird creature token with flying.
//!
//! Ascend (the special "if you control ten or more permanents" blessing)
//! is not in the usable keyword surface — GAP'd. The attack-token trigger
//! is wired; its "if you have the blessing" intervening-if gate has no
//! conditions:: predicate, so it is GAP'd and the trigger fires
//! unconditionally.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Windy City Aven");
    let bird = reg.interner_mut().intern("Bird");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(warrior);

    // GAP: keyword "Ascend MagicCon: Chicago" / city's-blessing is not a
    // usable KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            // GAP intervening-if: "if you have the Windy City's blessing" has
            // no conditions:: predicate; fires unconditionally.
            intervening_if: None,
            effect: make_bird,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_bird(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let bird = reg.interner().lookup("Bird").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: bird,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
