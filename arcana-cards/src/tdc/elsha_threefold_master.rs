//! Elsha, Threefold Master — `{U}{R}{W}` 1/1 Legendary Creature — Djinn Monk
//! with Trample.
//!
//! Oracle:
//! * Trample (base keyword).
//! * Prowess — NOT in the usable KeywordAbility surface; GAP'd.
//! * Whenever Elsha deals combat damage to a player, create that many 1/1
//!   white Monk creature tokens. (Wired; the count comes from the combat
//!   damage dealt. FIDELITY GAP: the tokens' own prowess keyword is not in the
//!   usable KeywordAbility surface, so the tokens are made without it.)

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elsha, Threefold Master");
    let djinn = reg.interner_mut().intern("Djinn");
    let monk = reg.interner_mut().intern("Monk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);
    subtypes.0.insert(monk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::creature(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: make_monks,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn make_monks(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    let monk = reg.interner().lookup("Monk").unwrap_or_default();
    let mut effects = Vec::new();
    for _ in 0..n {
        let mut subtypes = SubtypeSet::default();
        subtypes.0.insert(monk);
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: monk,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                // FIDELITY GAP: token's printed Prowess is not in the usable
                // KeywordAbility surface.
                keywords: vec![],
                abilities: vec![],
            },
        });
    }
    vec![Effect::Sequence(effects)]
}
