//! Crystal Dragon // Rob the Hoard — `{4}{W}{W}` / `{1}{W}` Adventure
//!
//! Creature: `{4}{W}{W}` Creature — Dragon (4/4)
//!   Flying, vigilance.
//!
//! Adventure: `{1}{W}` Sorcery — Rob the Hoard
//!   Return target artifact, enchantment, or legendary card from your
//!   graveyard to your hand.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crystal Dragon");
    let dragon_sub = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon_sub);
    let main_chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Rob the Hoard");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Return target artifact, enchantment, or legendary card from your graveyard to your hand.".into(),
        target_requirements: vec![TargetRequirement {
            filter: TargetFilter::Card {
                zone: Zone::Graveyard(0),
                filter: ObjectFilter::new()
                    .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::ENCHANTMENT))
                    .controlled_by(ControllerConstraint::You),
            },
            count: TargetCount::Exactly(1),
            controller: None,
        }],
        modal: None,
        effect: adv_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, main_chars).with_adventure(adventure))
}

fn adv_resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
