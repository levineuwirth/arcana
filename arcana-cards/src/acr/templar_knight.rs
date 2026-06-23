//! Templar Knight — `{1}{W}` 3/1 Creature — Human Knight.
//!
//! * Vigilance (keyword).
//! * "{W}, Tap five untapped attacking creatures you control named
//!   Templar Knight: Search your library for a legendary artifact card,
//!   put it onto the battlefield, then shuffle." — a mana + tap-five
//!   activation. The cost is wired via `tap_other` (filter: untapped
//!   attacking creatures you control named Templar Knight) with
//!   `tap_other_count: 5`; the effect tutors a legendary artifact card
//!   onto the battlefield (shuffle is automatic).
//! * "A deck can have any number of cards named Templar Knight." — a
//!   deckbuilding rule, not an in-game ability; nothing to wire.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Templar Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    // Cost filter: untapped attacking creatures you control named "Templar Knight".
    let self_name = reg.interner().lookup("Templar Knight");
    let tap_filter = ObjectFilter {
        name: self_name,
        ..ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .untapped_only()
            .attacking_only()
    };

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{W}, Tap five untapped attacking creatures you control named \
                   Templar Knight: Search your library for a legendary artifact \
                   card, put it onto the battlefield, then shuffle."
                .into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                tap_other: Some(tap_filter),
                tap_other_count: 5,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: tutor_legendary_artifact,
        }),
    )
}

fn tutor_legendary_artifact(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY));
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter,
        tapped: false,
    }]
}
