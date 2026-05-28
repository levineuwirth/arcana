//! Ratcatcher Trainee // Pest Problem — `{1}{R}` 2/1 red Human Peasant.
//! "During your turn, this creature has first strike." (static — GAP)
//! Adventure face "Pest Problem" (`{2}{R}` Instant):
//! "Create two 1/1 black Rat creature tokens with 'This token can't block.'"
//! GAP: "can't block" on token not expressible in TokenDefinition.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    CardDefinition, CardFace, CardRegistry, SpellAbilityDef,
};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ratcatcher Trainee");
    let human = reg.interner_mut().intern("Human");
    let peasant = reg.interner_mut().intern("Peasant");
    let _rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(peasant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: "during your turn, first strike" static conditional not expressible
        ..Default::default()
    };

    let adv_name = reg.interner_mut().intern("Pest Problem");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid adv cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create two 1/1 black Rat creature tokens with 'This token can't block.'".into(),
        target_requirements: Vec::new(),
        modal: None,
        effect: pest_problem,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(
        CardDefinition::new(name, chars).with_adventure(adventure),
    )
}

fn pest_problem(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let rat = reg.interner().lookup("Rat").expect("Rat interned during register()");
    let make_rat = || {
        let mut token_subtypes = SubtypeSet::default();
        token_subtypes.0.insert(rat);
        TokenDefinition {
            name: rat,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            // GAP: "can't block" ability not in TokenDefinition
            abilities: vec![],
        }
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: make_rat() },
        Effect::CreateToken { controller: entry.controller, token: make_rat() },
    ]
}
