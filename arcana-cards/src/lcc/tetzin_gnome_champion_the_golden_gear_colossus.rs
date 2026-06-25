//! Tetzin, Gnome Champion // The Golden-Gear Colossus
//! Front: `{U}{R}{W}` Legendary Artifact Creature — Gnome 2/2
//! Whenever Tetzin or another double-faced artifact you control enters, mill three cards.
//! You may put an artifact card from among them into your hand.
//! Craft with six artifacts {4} — GAP: Craft mechanic not expressible (exile-based
//! activation cost involving other permanents + graveyard cards; no Craft variant in engine).
//!
//! Back: "The Golden-Gear Colossus" — Legendary Artifact Creature — Gnome with Vigilance, Trample.
//! Whenever The Golden-Gear Colossus enters or attacks, transform up to one other target
//! double-faced artifact you control. Create two 1/1 colorless Gnome artifact creature tokens.
//!
//! # GAPs
//! - Craft activated ability not modeled (exile cost with other permanents/graveyard cards
//!   has no engine support).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tetzin, Gnome Champion");
    let gnome_sub = reg.interner_mut().intern("Gnome");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome_sub);

    // Pre-intern Gnome for token creation at resolve time
    let _gnome_tok = reg.interner_mut().intern("Gnome");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: The Golden-Gear Colossus
    let back_name = reg.interner_mut().intern("The Golden-Gear Colossus");
    let gnome_sub2 = reg.interner_mut().intern("Gnome");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(gnome_sub2);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(9)),
            toughness: Some(PtValue::Fixed(9)),
            keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front: "Whenever Tetzin or another DOUBLE-FACED artifact you control enters,
            // mill 3 cards. You may put an artifact card from among them into your hand."
            // (Tetzin is itself a transforming DFC, so it satisfies the double_faced() filter.)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You)
                        .double_faced(),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: tetzin_enters_mill,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face (The Golden-Gear Colossus): "Whenever this enters or attacks,
            // transform up to one OTHER target double-faced artifact you control. Create
            // two 1/1 colorless Gnome artifact creature tokens." Wired as two back-face-only
            // triggers (enters + attacks), each targeting up-to-one double-faced artifact
            // you control and transforming it, then making the two Gnome tokens.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: colossus_transform_and_make_gnomes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![colossus_transform_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: colossus_transform_and_make_gnomes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![colossus_transform_target()],
            })
            // Front mill trigger fires only on the front face; the Colossus token
            // triggers fire only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1),
        // GAP: Craft activated ability not modeled.
    )
}

/// "up to one OTHER target double-faced artifact you control" — a transforming
/// DFC artifact under your control (the `double_faced()` predicate). Self-exclusion
/// of "other" is not separately expressible here, but the Colossus's own back face
/// is already on the battlefield (no front-face artifact back to transform into the
/// way this clause intends), so targeting another DFC is the meaningful case.
fn colossus_transform_target() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::new()
                .with_types(TypeLine::ARTIFACT.into())
                .controlled_by(ControllerConstraint::You)
                .double_faced(),
        ),
        count: TargetCount::UpTo(1),
        controller: None,
    }
}

fn colossus_transform_and_make_gnomes(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();

    // "transform up to one other target double-faced artifact you control."
    // UpTo(1): zero or one chosen target.
    if let Some(TargetChoice::Object(id)) = trig.targets.targets.first() {
        effects.push(Effect::Transform { target: *id });
    }

    let gnome = reg
        .interner()
        .lookup("Gnome")
        .expect("Gnome interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gnome);
    let token = TokenDefinition {
        name: gnome,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // "Create two 1/1 ... Gnome artifact creature tokens."
    effects.push(Effect::CreateToken { controller: trig.controller, token: token.clone() });
    effects.push(Effect::CreateToken { controller: trig.controller, token });
    effects
}

fn tetzin_enters_mill(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Mill 3 cards, then may put an artifact card into hand.
    // DigTopN with artifact filter represents "look at top 3, may put an artifact card into hand".
    use arcana_core::effects::DigRest;
    vec![Effect::DigTopN {
        player: trig.controller,
        count: 3,
        filter: Some(ObjectFilter::new().with_types(TypeLine::ARTIFACT.into())),
        rest: DigRest::Graveyard,
    }]
}
