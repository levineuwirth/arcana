//! Kibo, Uktabi Prince — `{2}{G}` 2/2 Legendary Monkey Noble.
//!
//! {T}: Each player creates a colorless artifact token named Banana with a
//! sacrifice-for-mana ability.
//! Whenever an artifact an opponent controls is put into a graveyard from the
//! battlefield, put a +1/+1 counter on each Ape or Monkey you control.
//! Whenever Kibo attacks, defending player sacrifices an artifact of their
//! choice.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kibo, Uktabi Prince");
    let monkey = reg.interner_mut().intern("Monkey");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(monkey);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Each player creates a colorless artifact token named Banana with \"{T}, Sacrifice this token: Add {R} or {G}. You gain 2 life.\"".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_bananas,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::Opponent),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: counter_apes,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: defender_sacs_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_bananas(state: &GameState, _ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let banana = reg.interner().lookup("Banana").unwrap_or_default();
    // GAP: the Banana token's printed activated ability ("{T}, Sacrifice this
    // token: Add {R} or {G}. You gain 2 life.") is not expressible — token
    // definitions carry no custom activated abilities here. Bare token minted.
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::CreateToken {
            controller: p,
            token: TokenDefinition {
                name: banana,
                colors: ColorSet::colorless(),
                types: TypeLine::ARTIFACT.into(),
                subtypes: SubtypeSet::default(),
                power: None,
                toughness: None,
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}

fn counter_apes(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let ape = reg.interner().lookup("Ape").unwrap_or_default();
    let monkey = reg.interner().lookup("Monkey").unwrap_or_default();
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtypes_any(vec![ape, monkey]);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect()
}

fn defender_sacs_artifact(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(p) = trig.defending_player() else { return Vec::new(); };
    vec![Effect::Sacrifice {
        player: p,
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        count: 1,
    }]
}
