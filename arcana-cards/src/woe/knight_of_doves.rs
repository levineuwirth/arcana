//! Knight of Doves — `{2}{W}` 1/3 Creature — Human Knight.
//! "Whenever an enchantment you control is put into a graveyard from the battlefield, create a 1/1 white Bird creature token with flying."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of Doves");
    let human_sub = reg.interner_mut().intern("Human");
    let knight_sub = reg.interner_mut().intern("Knight");
    let _bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(knight_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                filter: ObjectFilter { types_any: Some(TypeLine::ENCHANTMENT.into()), ..Default::default() },
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
                intervening_if: None,
                effect: knight_of_doves_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn knight_of_doves_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let bird_tok = reg.interner().lookup("Bird").expect("Bird interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird_tok);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: bird_tok,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
