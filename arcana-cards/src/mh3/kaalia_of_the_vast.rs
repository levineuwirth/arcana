//! Kaalia of the Vast — `{1}{R}{W}{B}` 2/2 Legendary Human Cleric with
//! Flying. "Whenever Kaalia attacks an opponent, you may put an Angel,
//! Demon, or Dragon creature card from your hand onto the battlefield
//! tapped and attacking that opponent."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kaalia of the Vast");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    // Pre-intern the eligible put-from-hand subtypes so the resolver can
    // recover them by name.
    let _angel = reg.interner_mut().intern("Angel");
    let _demon = reg.interner_mut().intern("Demon");
    let _dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: put_fatty_attacking,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "You may put an Angel, Demon, or Dragon creature card from your hand
/// onto the battlefield tapped and attacking that opponent."
fn put_fatty_attacking(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut syms = Vec::new();
    if let Some(s) = reg.interner().lookup("Angel") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Demon") {
        syms.push(s);
    }
    if let Some(s) = reg.interner().lookup("Dragon") {
        syms.push(s);
    }
    let filter = ObjectFilter::creature().with_subtypes_any(syms);
    vec![Effect::PutFromHandOntoBattlefieldTappedAttacking {
        player: trig.controller,
        filter,
    }]
}
