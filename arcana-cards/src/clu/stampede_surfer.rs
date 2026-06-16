//! Stampede Surfer — `{3}{R/G}{R/G}` 4/4 red-green Human Warrior with Haste.
//! "Whenever this creature attacks, for each opponent, you create a 2/2 green
//! Boar creature token that's tapped and attacking that opponent."
//! (Fidelity GAP: the catalog has no create-token-tapped-attacking effect, so
//! the tokens are created normally — one per opponent — without the tapped /
//! attacking rider.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::KeywordAbility;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stampede Surfer");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    // Pre-intern the token subtype so the resolver can recover it.
    let _boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfAttacks,
            intervening_if: None,
            effect: on_attack_make_boars,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn on_attack_make_boars(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let boar = reg.interner().lookup("Boar").unwrap_or_default();
    let mut boar_subtypes = SubtypeSet::default();
    boar_subtypes.0.insert(boar);
    // One 2/2 green Boar per opponent (tapped-and-attacking rider is a GAP).
    let opps = script::opponents(state, trig.controller);
    opps.into_iter()
        .map(|_| Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: boar,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: boar_subtypes.clone(),
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}
