//! Shrill Howler // Howling Chorus — `{2}{G}` Werewolf Horror creature 3/1.
//! Front: Creatures with power less than this creature's power can't block it.
//!        {5}{G}: Transform this creature.
//! Back: Creatures with power less than this creature's power can't block it.
//!       Whenever this creature deals combat damage to a player, create a 3/2 colorless
//!       Eldrazi Horror creature token.
//!
//! GAP: "Creatures with power less than this creature's power can't block it" — static
//! ability restricting blockers based on dynamic power comparison is not expressible.
//! Not modeled.
//! GAP: back-face-only triggered ability not modeled (combat damage -> create 3/2 colorless
//! Eldrazi Horror token). The DamageDealt trigger fires on the back face only.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shrill Howler");
    let werewolf_sub = reg.interner_mut().intern("Werewolf");
    let horror_sub = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(werewolf_sub);
    subtypes.0.insert(horror_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: "Creatures with power less than this creature's power can't block it"
        // static evasion ability not expressible.
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Howling Chorus");
    let mut back_subtypes = SubtypeSet::default();
    let back_eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let back_werewolf_sub = reg.interner_mut().intern("Werewolf");
    back_subtypes.0.insert(back_eldrazi_sub);
    back_subtypes.0.insert(back_werewolf_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: same static blocker-restriction ability not modeled.
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face activated ability: {5}{G}: Transform this creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{G}: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{G}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(0), // front face only
                effect: transform_self,
            })
            // GAP: back-face-only triggered ability not modeled
            // (combat damage to player -> create 3/2 colorless Eldrazi Horror token)
    )
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
