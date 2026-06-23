//! Baru, Fist of Krosa — `{3}{G}{G}` 4/4 Legendary Creature — Human Druid. Green.
//!
//! Oracle text:
//! * Whenever a Forest enters, green creatures you control get +1/+1 and
//!   gain trample until end of turn.
//! * Grandeur — Discard another card named Baru, Fist of Krosa: Create an
//!   X/X green Wurm creature token, where X is the number of lands you
//!   control.
//!
//! Decomposition:
//! * No keyword line (Grandeur is an ability-word prefixing the activated
//!   ability, not a `KeywordAbility`).
//! * Forest-enters trigger → one `TriggeredAbilityDef` (ZoneChange on a
//!   Forest entering the battlefield) that pumps every green creature you
//!   control +1/+1 and grants Trample until end of turn via `ForEach` over
//!   `script::ids_matching` (a single Pump per id; Pump carries the granted
//!   keyword).
//! * Grandeur activated ability → discard another card named "Baru, Fist of
//!   Krosa" (cost = `discard_other` with a name filter; the engine always
//!   excludes this card), then create an X/X green Wurm where X = lands you
//!   control (dynamic token P/T computed at resolution).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Baru, Fist of Krosa");
    let human = reg.interner_mut().intern("Human");
    let druid = reg.interner_mut().intern("Druid");
    // Pre-intern the token subtype + the Forest subtype so the resolver /
    // the subtype filter can look them up read-only.
    let _wurm = reg.interner_mut().intern("Wurm");
    let _forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // "Whenever a Forest enters" — a Forest-subtyped land entering.
    let forest_filter = script::subtype_filter(reg, "Forest");

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: forest_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: pump_green_creatures,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Grandeur — Discard another card named Baru, Fist of Krosa: Create an X/X green Wurm creature token, where X is the number of lands you control.".into(),
                cost: ActivationCost {
                    discard_other: Some(ObjectFilter {
                        name: Some(name),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_wurm,
            }),
    )
}

fn pump_green_creatures(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids = script::ids_matching(
        state,
        &ObjectFilter::creature()
            .controlled_by(ControllerConstraint::You)
            .with_colors(ColorSet::green()),
        trig.controller,
    );
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Trample],
        }),
    }]
}

fn make_wurm(state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let x = script::count_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        ctx.controller,
    );
    let wurm = reg.interner().lookup("Wurm").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: wurm,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(x as i32)),
            toughness: Some(PtValue::Fixed(x as i32)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
