//! Suki, Kyoshi Warrior — `{2}{G/W}{G/W}` */4 Legendary Creature — Human Warrior Ally.
//! Suki's power is equal to the number of creatures you control.
//! Whenever Suki attacks, create a 1/1 white Ally creature token that's tapped
//! and attacking.
//!
//! "Suki's power is equal to the number of creatures you control" is a
//! self-CDA wired at Layer 7a via `ContinuousEffect::self_pt_cda` on an ETB
//! trigger; the bones record power as PtValue::Star (*) and toughness stays
//! the printed fixed 4 (so the asymmetric CDA can't use self_pt_from_match).

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suki, Kyoshi Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let ally = reg.interner_mut().intern("Ally");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);
    subtypes.0.insert(ally);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_make_ally,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "Suki's power is equal to the number of creatures you control" — install
/// the self-CDA at Layer 7a; toughness stays the printed 4.
fn install_cda(_s: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_cda(
            trig.source,
            cda_pt,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

// Power = number of creatures you control; toughness = 4 (printed).
fn cda_pt(s: &GameState, source: ObjectId) -> (i32, i32) {
    let who = s.objects.get(source).map(|o| o.controller).unwrap_or(0);
    let n = s
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == who && o.characteristics.types.is_creature())
        .count() as i32;
    (n, 4)
}

fn attack_make_ally(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let ally = match reg.interner().lookup("Ally") {
        Some(s) => s,
        None => return Vec::new(),
    };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ally);
    vec![Effect::CreateTokenTappedAttacking {
        controller: trig.controller,
        token: TokenDefinition {
            name: ally,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
