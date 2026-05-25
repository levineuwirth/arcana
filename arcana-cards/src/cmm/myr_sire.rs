//! Myr Sire — `{2}` 1/1 Artifact Creature — Phyrexian Myr.
//! "When this creature dies, create a 1/1 colorless Phyrexian Myr artifact creature token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Myr Sire");
    let phyrexian_sub = reg.interner_mut().intern("Phyrexian");
    let myr_sub = reg.interner_mut().intern("Myr");
    let _myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian_sub);
    subtypes.0.insert(myr_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: myr_sire_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn myr_sire_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let myr_tok = reg.interner().lookup("Myr").expect("Myr interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr_tok);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: myr_tok,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
