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
//! The back-face combat-damage trigger ("whenever this creature deals combat damage
//! to a player, create a 3/2 colorless Eldrazi Horror token") is wired as a
//! DamageDealt trigger restricted to this creature (by name) and gated to face 1.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

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
            // Back face (Howling Chorus): whenever this creature deals combat damage
            // to a player, create a 3/2 colorless Eldrazi Horror creature token.
            // Restricted to this creature by its back-face name; face-gated to face 1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter {
                        name: Some(back_name),
                        ..ObjectFilter::default()
                    },
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: create_eldrazi_horror,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

/// Back face: create a 3/2 colorless Eldrazi Horror creature token.
fn create_eldrazi_horror(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi = reg.interner().lookup("Eldrazi").expect("Eldrazi interned during register()");
    let horror = reg.interner().lookup("Horror").expect("Horror interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(horror);
    let token = TokenDefinition {
        // Nameless creature token; use its "Eldrazi" subtype symbol as the
        // display name (matches the surrounding token-creation idiom).
        name: eldrazi,
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn transform_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}
