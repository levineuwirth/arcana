//! Cadira, Caller of the Small — `{1}{G}{W}` 3/3 Legendary Creature —
//! Orc Ranger. G/W. Trample.
//! "Whenever Cadira deals combat damage to a player, for each token you
//! control, create a 1/1 white Rabbit creature token." — DamageDealt
//! (you, to a player, combat) → ForEach over the tokens you control,
//! minting one Rabbit per token.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cadira, Caller of the Small");
    let orc = reg.interner_mut().intern("Orc");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);
    subtypes.0.insert(ranger);
    // pre-intern Rabbit token subtype
    let _rabbit = reg.interner_mut().intern("Rabbit");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::new().controlled_by(ControllerConstraint::You),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: create_rabbit_per_token,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn create_rabbit_per_token(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let token_ids = script::ids_matching(
        state,
        &ObjectFilter::permanent()
            .controlled_by(ControllerConstraint::You)
            .tokens_only(),
        trig.controller,
    );
    let rabbit = reg.interner().lookup("Rabbit").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rabbit);
    vec![Effect::ForEach {
        targets: token_ids,
        effect: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: rabbit,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        }),
    }]
}
