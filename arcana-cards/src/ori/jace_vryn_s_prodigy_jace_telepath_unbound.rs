//! Jace, Vryn's Prodigy // Jace, Telepath Unbound — `{1}{U}` Legendary Creature
//! — Human Wizard 0/2 (front). Activated ability: `{T}: Draw a card, then
//! discard a card. If there are five or more cards in your graveyard, exile
//! Jace, then return him to the battlefield transformed under his owner's
//! control.`
//!
//! Back face: Legendary Planeswalker — Jace (loyalty 5).
//! +1: Up to one target creature gets -2/-0 until your next turn.
//! -3: You may cast target instant or sorcery card from your graveyard this
//!     turn. If that spell would be put into your graveyard, exile it instead.
//! -9: You get an emblem with "Whenever you cast a spell, target opponent mills
//!     five cards."
//!
//! GAP: front-face loot/exile/return-transformed needs a zone-change continuation.
//! GAP: −3 needs casting permission (including alternative costs), not flashback.
//! GAP: −9 needs correct controller identity when choosing an emblem trigger’s targets.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jace, Vryn's Prodigy");
    let human_sub = reg.interner_mut().intern("Human");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Legendary Planeswalker — Jace with starting loyalty 5.
    let back_name = reg.interner_mut().intern("Jace, Telepath Unbound");
    let jace_sub = reg.interner_mut().intern("Jace");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(jace_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(5),
            ..Default::default()
        },
        spell_ability: None,
    };

    let mut target = TargetRequirement::target_creature();
    target.count = TargetCount::UpTo(1);
    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Up to one target creature gets -2/-0 until your next turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..Default::default()
                },
                target_requirements: vec![target],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: plus_one,
            }),
    )
}

fn plus_one(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return vec![];
    };
    vec![Effect::Pump {
        target: *id,
        power: -2,
        toughness: 0,
        keywords: vec![],
        duration: Duration::UntilYourNextTurn(ctx.controller),
    }]
}
