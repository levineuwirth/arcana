//! Ultra Magnus, Tactician // Ultra Magnus, Armored Carrier (transforming DFC, layout "transform")
//!
//! Front face: Ultra Magnus, Tactician — {4}{R}{G}{W} Legendary Artifact Creature — Robot, 7/7 (G/R/W).
//!   More Than Meets the Eye {2}{R}{G}{W}.
//!   Ward {2}.
//!   Whenever Ultra Magnus attacks, you may put an artifact creature card from your hand onto
//!     the battlefield tapped and attacking. If you do, convert Ultra Magnus at end of combat.
//! Back face: Ultra Magnus, Armored Carrier — Legendary Artifact — Vehicle (G/R/W).
//!   Living metal. Haste.
//!   Formidable — Whenever Ultra Magnus attacks, attacking creatures you control gain
//!     indestructible until end of turn. If those creatures have total power 8 or greater,
//!     convert Ultra Magnus.
//!
//! GAP: "More Than Meets the Eye" / "Living metal" / "Formidable" / "Convert" are not in the
//!   usable keyword surface — only Haste and Ward {2} are emitted. Both attack triggers are
//!   GAPs: the front trigger puts a card from hand onto the battlefield tapped-and-attacking
//!   (no Effect for "put from hand onto battlefield attacking") and conditionally converts at
//!   end of combat (no delayed-transform); the back Formidable trigger grants indestructible to
//!   the dynamic set "attacking creatures you control" (no script filter for the attacking set)
//!   and converts on a total-power threshold. "Convert" is this card's transform — there is no
//!   transform-completion or end-of-combat-conditional-transform hook to author it cleanly.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ultra Magnus, Tactician");
    let robot = reg.interner_mut().intern("Robot");
    let vehicle = reg.interner_mut().intern("Vehicle");

    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost"))],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Ultra Magnus, Armored Carrier");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
            types: TypeLine(TypeLine::ARTIFACT),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
