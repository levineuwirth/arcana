//! Nissa, Vastwood Seer // Nissa, Sage Animist — `{2}{G}` Legendary Creature — Elf Scout // Legendary Planeswalker — Nissa.
//! Front: 2/2.
//!   When Nissa enters, you may search your library for a basic Forest card,
//!   reveal it, put it into your hand, then shuffle.
//!   Whenever a land you control enters, if you control seven or more lands,
//!   exile Nissa, then return her to the battlefield transformed under her
//!   owner's control.
//! ---
//! Back (Nissa, Sage Animist): Legendary Planeswalker — Nissa (starting loyalty 3).
//!   +1: Reveal the top card of your library. If it's a land card, put it onto
//!       the battlefield. Otherwise, put it into your hand.
//!   −2: Create Ashaya, the Awoken World, a legendary 4/4 green Elemental creature token.
//!   −7: Untap up to six target lands. They become 6/6 Elemental creatures.
//!       They're still lands.
//!
//! GAP: "exile Nissa, then return her to the battlefield transformed" — no
//!      ExileAndReturnTransformed effect; using Effect::Transform as approximation.
//! "if you control seven or more lands" — intervening-if condition on land count
//!      modeled via `conditions::you_control_at_least` on `intervening_if`.
//! GAP: −7 "lands become 6/6 Elemental creatures; they're still lands" — the "type
//!      change + P/T setting while retaining Land" continuous effect is not in engine.
//!      The untap portion is modeled; the type-change animation is GAP.
//! GAP: back-face-only loyalty abilities not auto-installed on transform (engine debt).
//! GAP: Ashaya token "legendary" supertype — TokenDefinition has no supertypes field.

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Vastwood Seer");
    let elf_sub = reg.interner_mut().intern("Elf");
    let scout_sub = reg.interner_mut().intern("Scout");
    let forest_sub = reg.interner_mut().intern("Forest");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let _ashaya = reg.interner_mut().intern("Ashaya, the Awoken World");

    let mut front_subs = SubtypeSet::default();
    front_subs.0.insert(elf_sub);
    front_subs.0.insert(scout_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subs,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // Back face: Nissa, Sage Animist — Legendary Planeswalker — Nissa (loyalty 3)
    let back_name = reg.interner_mut().intern("Nissa, Sage Animist");
    let nissa_sub = reg.interner_mut().intern("Nissa");
    let mut back_subs = SubtypeSet::default();
    back_subs.0.insert(nissa_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine::PLANESWALKER.into(),
            subtypes: back_subs,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            loyalty: Some(3),
            ..Default::default()
        },
        spell_ability: None,
    };

    // Land filter for ZoneChange trigger (lands you control enter)
    let land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // When Nissa enters, you may search your library for a basic Forest card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_tutor_forest,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Whenever a land you control enters, if you control 7+ lands,
            // transform Nissa.
            // GAP: "exile, then return transformed" — using Transform as approximation.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: land_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                // Intervening-if "if you control seven or more lands" via conditions::you_control_at_least.
                intervening_if: Some(iif_seven_lands),
                effect: on_land_enters_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: +1 loyalty ability — reveal top card; land ETB else hand.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Reveal the top card of your library. If it's a land card, put it onto the battlefield. Otherwise, put it into your hand.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: plus_one_reveal,
            })
            // Back face: −2 loyalty ability — create Ashaya token.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create Ashaya, the Awoken World, a legendary 4/4 green Elemental creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: minus_two_ashaya,
            })
            // Back face: −7 loyalty ability — untap up to 6 lands (type change GAP).
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Untap up to six target lands. They become 6/6 Elemental creatures. They're still lands. (GAP: type change not modeled)".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::UpTo(6),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: minus_seven_untap_lands,
            }),
    )
}

fn etb_tutor_forest(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let forest = reg.interner().lookup("Forest");
    let filter = ObjectFilter {
        subtypes: forest.map(|f| vec![f]),
        supertypes: Some(SupertypeSet(SupertypeSet::BASIC)),
        types: Some(TypeLine::LAND.into()),
        ..ObjectFilter::default()
    };
    vec![Effect::TutorToHand {
        player: trig.controller,
        filter,
        reveal: true,
    }]
}

fn iif_seven_lands(state: &GameState, _source: ObjectId, you: PlayerId) -> bool {
    conditions::you_control_at_least(state, you, &ObjectFilter::new().with_types(TypeLine::LAND.into()), 7)
}

fn on_land_enters_transform(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Intervening if: you control seven or more lands.
    let land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    let land_count = script::count_matching(state, &land_filter, trig.controller);
    if land_count >= 7 {
        // GAP: "exile, then return transformed" — using Transform as approximation.
        vec![Effect::Transform { target: trig.source }]
    } else {
        Vec::new()
    }
}

fn plus_one_reveal(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    use arcana_core::effects::{DigRest, RevealDest};
    // Reveal top card; if land → battlefield, else → hand.
    // RevealUntil with max_reveal: Some(1) models "look at the top card".
    // found_dest: Battlefield for a land (filter: land only).
    // For the non-land case we need DigTopN. Use RevealUntil for the land case:
    vec![
        Effect::RevealUntil {
            player: ctx.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            found_dest: RevealDest::Battlefield,
            rest: DigRest::BottomRandom,
            max_reveal: Some(1),
        },
    ]
    // GAP: if the top card is a non-land, RevealUntil will fail to find a match and
    //      put it on the bottom instead of into hand. The dual case (land→battlefield,
    //      nonland→hand) requires a conditional Effect not expressible in one variant.
}

fn minus_two_ashaya(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elemental = reg.interner().lookup("Elemental").expect("Elemental interned at register");
    let ashaya_name = reg.interner().lookup("Ashaya, the Awoken World").unwrap_or(elemental);
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(elemental);
    let token = TokenDefinition {
        name: ashaya_name,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
        // GAP: legendary supertype not modeled on TokenDefinition.
    };
    vec![Effect::CreateToken { controller: ctx.controller, token }]
}

fn minus_seven_untap_lands(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Untap up to six target lands.
    // GAP: "They become 6/6 Elemental creatures. They're still lands." —
    //      type-change continuous effect not expressible in current engine.
    let mut effects = Vec::new();
    for target in &ctx.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::Untap { target: *id });
            // GAP: SetBasePT + type-line becoming Creature/Land simultaneously not expressible.
        }
    }
    effects
}
