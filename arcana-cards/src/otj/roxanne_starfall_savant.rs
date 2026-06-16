//! Roxanne, Starfall Savant — `{3}{R}{G}` 4/3 Legendary Cat Druid.
//! Whenever Roxanne enters or attacks, create a tapped colorless artifact
//! token named Meteorite (its printed ETB-damage and tap-for-mana abilities
//! are a token-fidelity gap). Whenever you tap an artifact token for mana,
//! add one mana of any type that artifact token produced (GAP — no matching
//! trigger condition / mana-replication effect).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roxanne, Starfall Savant");
    let cat = reg.interner_mut().intern("Cat");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: make_meteorite,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: make_meteorite,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: trigger — "Whenever you tap an artifact token for mana, add one
        // mana of any type that artifact token produced": no trigger condition
        // for tapping-for-mana, and no mana-replication effect.
    )
}

fn make_meteorite(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_name = reg
        .interner()
        .lookup("Meteorite")
        .unwrap_or_default();
    // Token's printed ETB-damage and {T}: add-any-color abilities are a
    // token-fidelity gap; mints the bare colorless artifact token (tapped
    // status is also unmodeled in plain CreateToken).
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: token_name,
            colors: ColorSet::colorless(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: SubtypeSet::default(),
            power: None,
            toughness: None,
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
