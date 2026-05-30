//! Zenos yae Galvus // Shinryu, Transcendent Rival
//!
//! Front (Legendary Creature — Human Noble Warrior 4/4, {3}{B}{B}):
//!   My First Friend — When Zenos yae Galvus enters, choose a creature an opponent controls.
//!   Until end of turn, creatures other than Zenos and the chosen creature get -2/-2.
//!   When the chosen creature leaves the battlefield, transform Zenos yae Galvus.
//! Back (Legendary Creature — Dragon):
//!   Flying.
//!   As this transforms into Shinryu, choose an opponent.
//!   Burning Chains — When the chosen player loses the game, you win the game.
//!
//! GAP: ETB "choose a creature an opponent controls" — the choice/remembering mechanic is not
//!       modeled. Using a ForEach over opponent creatures with Pump(-2/-2) as an approximation
//!       would be wrong (it affects ALL, not all-except-two). Whole ETB effect GAP'd.
//! GAP: "When the chosen creature leaves the battlefield" — requires storing a specific
//!       chosen object id as state; not modelable. Transform trigger GAP'd.
//! GAP: "As this transforms into Shinryu, choose an opponent" — on-transform choice not modeled.
//! GAP: "When the chosen player loses the game, you win the game" — win condition trigger
//!       not modeled (engine has no GameEvent::PlayerLost).
//! GAP: back-face-only triggered abilities not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zenos yae Galvus");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Shinryu, Transcendent Rival");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dragon);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
