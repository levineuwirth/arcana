//! Plant a Sapling // Fully-Grown Treefolk — `{G}` transforming DFC.
//! Front (Plant a Sapling — Sorcery):
//!   Search your library for a basic land card, reveal it, put it into your hand,
//!   then shuffle this spell into its owner's library transformed.
//! Back (Fully-Grown Treefolk — Creature — Treefolk, */*):
//!   Fully-Grown Treefolk's power and toughness are each equal to the number of
//!   lands you control.
//!
//! GAPs:
//! - Front face "then shuffle this spell into its owner's library transformed":
//!   a spell shuffling ITSELF into the library (rather than going to the graveyard)
//!   AND flipping its transform state is not expressible — no Effect for "shuffle
//!   this spell into library transformed". The tutor-to-hand half is modeled.
//! - Back face P/T "*/* equal to the number of lands you control" is a
//!   characteristic-defining ability. Emitted as PtValue::Star (printed `*`); the
//!   CDA that sets it to the land count is not wireable with the demonstrated API.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Plant a Sapling");
    let treefolk = reg.interner_mut().intern("Treefolk");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Fully-Grown Treefolk");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(treefolk);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: */* CDA equal to lands you control; emitted as printed `*`.
            power: Some(PtValue::Star),
            toughness: Some(PtValue::Star),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a basic land card, reveal it, put it into \
                       your hand, then shuffle this spell into its owner's library \
                       transformed."
                    .into(),
                target_requirements: Vec::new(),
                modal: None,
                effect: resolve_front,
            }),
    )
}

fn resolve_front(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Search your library for a basic land card; reveal it; put it into your hand.
    let basic_land = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC));
    // GAP: "then shuffle this spell into its owner's library transformed" — not
    // expressible; the spell will go to the graveyard normally instead.
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: basic_land,
        reveal: true,
    }]
}
