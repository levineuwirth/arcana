//! Liliana, Heretical Healer // Liliana, Defiant Necromancer
//!
//! Front face — Legendary Creature — Human Cleric, 2/3, Lifelink.
//! Whenever another nontoken creature you control dies, exile Liliana, Heretical
//! Healer, then return her to the battlefield transformed under her owner's
//! control. If you do, create a 2/2 black Zombie creature token.
//!
//! Back face — Legendary Planeswalker — Liliana, starting loyalty 3:
//!   +2: Each player discards a card.
//!   -X: Return target nonlegendary creature card with mana value X from your
//!       graveyard to the battlefield.
//!   -8: You get an emblem with "Whenever a creature dies, return it to the
//!       battlefield under your control at the beginning of the next end step."
//!
//! GAP: the front death-trigger's faithful "exile then return transformed" is
//! approximated by Effect::Transform (the creature flips in place rather than
//! exile/return); the Zombie token is created. Planeswalker loyalty abilities
//! are not an expressible ability shape in this catalog, so the back face's
//! +2 / -X / -8 abilities are GAPed (the face exists with loyalty 3 but its
//! activated loyalty abilities are not wired).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana, Heretical Healer");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    // Back face — Legendary Planeswalker — Liliana, loyalty 3.
    let back_name = reg.interner_mut().intern("Liliana, Defiant Necromancer");
    let liliana_sub = reg.interner_mut().intern("Liliana");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(liliana_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(3),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face death trigger.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_creature_dies,
                trigger_zones: Vec::new(),
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0),
    )
}

fn on_creature_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let zombie = reg.interner().lookup("Zombie");
    let mut subtypes = SubtypeSet::default();
    if let Some(z) = zombie {
        subtypes.0.insert(z);
    }
    // GAP: faithful "exile Liliana, then return her transformed" is approximated
    // by an in-place transform of the source permanent.
    vec![
        Effect::Transform { target: trig.source },
        Effect::CreateToken {
            controller: trig.controller,
            token: arcana_core::effects::TokenDefinition {
                name: zombie.unwrap_or_default(),
                colors: ColorSet::black(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
