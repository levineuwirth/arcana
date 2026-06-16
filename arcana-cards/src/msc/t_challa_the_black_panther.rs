//! T'Challa, the Black Panther — `{1}{G}{W}` 2/2 Legendary Creature — Human Noble Hero.
//!
//! * Whenever T'Challa enters or attacks, create a tapped Vibranium token.
//!   (Modeled as two triggers — SelfEntersBattlefield + SelfAttacks — each
//!   minting an indestructible artifact token. The "tapped" entry and the
//!   token's "{T}: Add {C}, restricted to artifact spells" activated ability
//!   are not expressible — see GAPs.)
//! * Whenever you cast an artifact spell with mana value 4 or greater, put
//!   two +1/+1 counters on T'Challa.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::targets::ObjectFilter;
use arcana_core::zones::Zone;
use arcana_core::effects::TokenDefinition;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("T'Challa, the Black Panther");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let hero = reg.interner_mut().intern("Hero");
    let _vibranium = reg.interner_mut().intern("Vibranium");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);
    subtypes.0.insert(hero);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_vibranium,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_vibranium,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .with_min_cmc(4),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_two_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn make_vibranium(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the token enters TAPPED (no untargeted CreateToken-tapped variant)
    // and its "{T}: Add {C} (artifact-only)" activated ability cannot be
    // attached to a TokenDefinition here. Emit the indestructible artifact
    // token body as best-effort.
    let nm = reg.interner().lookup("Vibranium").unwrap_or_default();
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: nm,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![KeywordAbility::Indestructible],
            abilities: vec![],
        },
    }]
}

fn add_two_counters(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}
