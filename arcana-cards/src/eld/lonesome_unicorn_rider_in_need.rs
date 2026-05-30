//! Lonesome Unicorn // Rider in Need — `{4}{W}` Unicorn creature 3/3 with
//! Vigilance; Adventure face "Rider in Need" (`{2}{W}` sorcery, create a
//! 2/2 white Knight creature token with vigilance).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lonesome Unicorn");
    let unicorn_sub = reg.interner_mut().intern("Unicorn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(unicorn_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // Pre-intern "Knight" so the resolver can look it up.
    let _knight_sub = reg.interner_mut().intern("Knight");

    // Adventure face "Rider in Need" — sorcery {2}{W}, create a 2/2 white
    // Knight creature token with vigilance.
    let adv_name = reg.interner_mut().intern("Rider in Need");
    let adv_chars = Characteristics {
        name: adv_name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid adv cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    let adv_ability = SpellAbilityDef {
        text: "Create a 2/2 white Knight creature token with vigilance.".into(),
        target_requirements: vec![],
        modal: None,
        effect: rider_in_need_resolve,
    };
    let adventure = CardFace {
        name: adv_name,
        characteristics: adv_chars,
        spell_ability: Some(adv_ability),
    };

    reg.register(CardDefinition::new(name, chars).with_adventure(adventure))
}

fn rider_in_need_resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let knight = reg.interner().lookup("Knight")
        .expect("Knight interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(knight);
    vec![Effect::CreateToken {
        controller: entry.controller,
        token: TokenDefinition {
            name: knight,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            keywords: vec![KeywordAbility::Vigilance],
            abilities: vec![],
        },
    }]
}
